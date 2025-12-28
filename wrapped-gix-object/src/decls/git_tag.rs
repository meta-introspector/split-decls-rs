macro_rules! deps {
    () => {
        TagRef!();
        Kind!();
    };
}

macro_rules! git_tag {
    () => {
        deps!();
        pub fn git_tag < 'a , E : ParserError < & 'a [u8] > + AddContext < & 'a [u8] , StrContext > > (i : & mut & 'a [u8] ,) -> ModalResult < TagRef < 'a > , E > { ((| i : & mut _ | parse :: header_field (i , b"object" , parse :: hex_hash)) . context (StrContext :: Expected ("object <40 lowercase hex char>" . into ())) , (| i : & mut _ | parse :: header_field (i , b"type" , take_while (1 .. , AsChar :: is_alpha))) . verify_map (| kind | crate :: Kind :: from_bytes (kind) . ok ()) . context (StrContext :: Expected ("type <object kind>" . into ())) , (| i : & mut _ | parse :: header_field (i , b"tag" , take_while (1 .. , | b | b != NL [0]))) . context (StrContext :: Expected ("tag <version>" . into ())) , opt (| i : & mut _ | parse :: header_field (i , b"tagger" , parse :: signature_and_consumed) . map (| (_signature , raw) | raw)) . context (StrContext :: Expected ("tagger <signature>" . into ())) , terminated (message , eof) ,) . map (| (target , kind , tag_version , tagger , (message , pgp_signature)) | TagRef { target , name : tag_version . as_bstr () , target_kind : kind , message , tagger , pgp_signature , }) . parse_next (i) }
    };
}

git_tag!();