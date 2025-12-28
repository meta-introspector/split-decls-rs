macro_rules! deps {
    () => {
        CommitRefIter!();
        Tree!();
        ExtraHeaders!();
        Error!();
        State!();
        SignatureKind!();
        Token!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < 'a > CommitRefIter < 'a > { # [inline] fn next_inner (mut i : & 'a [u8] , state : & mut State) -> Result < (& 'a [u8] , Token < 'a >) , crate :: decode :: Error > { let input = & mut i ; match Self :: next_inner_ (input , state) { Ok (token) => Ok ((* input , token)) , Err (err) => Err (crate :: decode :: Error :: with_err (err , input)) , } } fn next_inner_ (input : & mut & 'a [u8] , state : & mut State ,) -> Result < Token < 'a > , winnow :: error :: ErrMode < crate :: decode :: ParseError > > { use State :: * ; Ok (match state { Tree => { let tree = (| i : & mut _ | parse :: header_field (i , b"tree" , parse :: hex_hash)) . context (StrContext :: Expected ("tree <40 lowercase hex char>" . into ())) . parse_next (input) ? ; * state = State :: Parents ; Token :: Tree { id : ObjectId :: from_hex (tree) . expect ("parsing validation") , } } Parents => { let parent = opt (| i : & mut _ | parse :: header_field (i , b"parent" , parse :: hex_hash)) . context (StrContext :: Expected ("commit <40 lowercase hex char>" . into ())) . parse_next (input) ? ; match parent { Some (parent) => Token :: Parent { id : ObjectId :: from_hex (parent) . expect ("parsing validation") , } , None => { * state = State :: Signature { of : SignatureKind :: Author , } ; Self :: next_inner_ (input , state) ? } } } Signature { ref mut of } => { let who = * of ; let (field_name , err_msg) = match of { SignatureKind :: Author => { * of = SignatureKind :: Committer ; (& b"author" [..] , "author <signature>") } SignatureKind :: Committer => { * state = State :: Encoding ; (& b"committer" [..] , "committer <signature>") } } ; let signature = (| i : & mut _ | parse :: header_field (i , field_name , parse :: signature)) . context (StrContext :: Expected (err_msg . into ())) . parse_next (input) ? ; match who { SignatureKind :: Author => Token :: Author { signature } , SignatureKind :: Committer => Token :: Committer { signature } , } } Encoding => { let encoding = opt (| i : & mut _ | parse :: header_field (i , b"encoding" , take_till (0 .. , NL))) . context (StrContext :: Expected ("encoding <encoding>" . into ())) . parse_next (input) ? ; * state = State :: ExtraHeaders ; match encoding { Some (encoding) => Token :: Encoding (encoding . as_bstr ()) , None => Self :: next_inner_ (input , state) ? , } } ExtraHeaders => { let extra_header = opt (alt ((| i : & mut _ | parse :: any_header_field_multi_line (i) . map (| (k , o) | (k . as_bstr () , Cow :: Owned (o))) , | i : & mut _ | { parse :: any_header_field (i , take_till (0 .. , NL)) . map (| (k , o) | (k . as_bstr () , Cow :: Borrowed (o . as_bstr ()))) } ,))) . context (StrContext :: Expected ("<field> <single-line|multi-line>" . into ())) . parse_next (input) ? ; match extra_header { Some (extra_header) => Token :: ExtraHeader (extra_header) , None => { * state = State :: Message ; Self :: next_inner_ (input , state) ? } } } Message => { let message = terminated (decode :: message , eof) . parse_next (input) ? ; debug_assert ! (input . is_empty () , "we should have consumed all data - otherwise iter may go forever") ; Token :: Message (message) } }) } }
    };
}

impl_34!()