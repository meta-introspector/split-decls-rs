macro_rules! fix {
    () => {
        fn fix (id : & 'static str , label : & str , source_change : SourceChange , target : TextRange) -> Assist { let mut res = unresolved_fix (id , label , target) ; res . source_change = Some (source_change) ; res }
    };
}

fix!()