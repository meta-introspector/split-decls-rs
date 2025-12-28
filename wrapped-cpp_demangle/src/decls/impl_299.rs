macro_rules! deps {
    () => {
        CallOffset!();
        SpecialName!();
        ParseContext!();
        Name!();
        Encoding!();
        Result!();
        ResourceName!();
        IndexStr!();
        SubstitutionTable!();
        Parse!();
        SeqId!();
        Error!();
    };
}

macro_rules! impl_299 {
    () => {
        deps!();
        impl Parse for SpecialName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (SpecialName , IndexStr < 'b >) > { try_begin_parse ! ("SpecialName" , ctx , input) ; let (head , tail) = match input . try_split_at (2) { None => return Err (error :: Error :: UnexpectedEnd) , Some ((head , tail)) => (head , tail) , } ; match head . as_ref () { b"TV" => { let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; Ok ((SpecialName :: VirtualTable (ty) , tail)) } b"TT" => { let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; Ok ((SpecialName :: Vtt (ty) , tail)) } b"TI" => { let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; Ok ((SpecialName :: Typeinfo (ty) , tail)) } b"TS" => { let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; Ok ((SpecialName :: TypeinfoName (ty) , tail)) } b"Tc" => { let (first , tail) = CallOffset :: parse (ctx , subs , tail) ? ; let (second , tail) = CallOffset :: parse (ctx , subs , tail) ? ; let (base , tail) = Encoding :: parse (ctx , subs , tail) ? ; Ok ((SpecialName :: VirtualOverrideThunkCovariant (first , second , Box :: new (base)) , tail ,)) } b"Th" | b"Tv" => { let tail = consume (b"T" , input) . unwrap () ; let (offset , tail) = CallOffset :: parse (ctx , subs , tail) ? ; let (base , tail) = Encoding :: parse (ctx , subs , tail) ? ; Ok ((SpecialName :: VirtualOverrideThunk (offset , Box :: new (base)) , tail ,)) } b"TC" => { let (ty1 , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; let (n , tail) = parse_number (10 , false , tail) ? ; let tail = consume (b"_" , tail) ? ; let (ty2 , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; Ok ((SpecialName :: ConstructionVtable (ty1 , n as usize , ty2) , tail)) } b"TF" => { let (ty , tail) = TypeHandle :: parse (ctx , subs , tail) ? ; Ok ((SpecialName :: TypeinfoFunction (ty) , tail)) } b"TH" => { let (name , tail) = Name :: parse (ctx , subs , tail) ? ; Ok ((SpecialName :: TlsInit (name) , tail)) } b"TW" => { let (name , tail) = Name :: parse (ctx , subs , tail) ? ; Ok ((SpecialName :: TlsWrapper (name) , tail)) } b"GV" => { let (name , tail) = Name :: parse (ctx , subs , tail) ? ; Ok ((SpecialName :: Guard (name) , tail)) } b"GR" => { let (name , tail) = Name :: parse (ctx , subs , tail) ? ; let (idx , tail) = if let Ok (tail) = consume (b"_" , tail) { (0 , tail) } else { let (idx , tail) = SeqId :: parse (ctx , subs , tail) ? ; let tail = consume (b"_" , tail) ? ; (idx . 0 + 1 , tail) } ; Ok ((SpecialName :: GuardTemporary (name , idx) , tail)) } b"Gr" => { let (resource_name_len , tail) = parse_number (10 , false , tail) ? ; if resource_name_len == 0 { return Err (error :: Error :: UnexpectedText) ; } let (head , tail) = match tail . try_split_at (resource_name_len as _) { Some ((head , tail)) => (head , tail) , None => return Err (error :: Error :: UnexpectedEnd) , } ; let head = consume (b"_" , head) ? ; let (resource_names , empty) = zero_or_more :: < ResourceName > (ctx , subs , head) ? ; if ! empty . is_empty () { return Err (error :: Error :: UnexpectedText) ; } Ok ((SpecialName :: JavaResource (resource_names) , tail)) } b"GT" => { match tail . next_or (error :: Error :: UnexpectedEnd) ? { (b'n' , tail) => { let (base , tail) = Encoding :: parse (ctx , subs , tail) ? ; Ok ((SpecialName :: NonTransactionClone (Box :: new (base)) , tail)) } (b't' , tail) | (_ , tail) => { let (base , tail) = Encoding :: parse (ctx , subs , tail) ? ; Ok ((SpecialName :: TransactionClone (Box :: new (base)) , tail)) } } } _ => Err (error :: Error :: UnexpectedText) , } } }
    };
}

impl_299!()