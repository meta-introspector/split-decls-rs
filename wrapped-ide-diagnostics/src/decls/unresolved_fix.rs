macro_rules! unresolved_fix {
    () => {
        fn unresolved_fix (id : & 'static str , label : & str , target : TextRange) -> Assist { assert ! (! id . contains (' ')) ; Assist { id : AssistId :: quick_fix (id) , label : Label :: new (label . to_owned ()) , group : None , target , source_change : None , command : None , } }
    };
}

unresolved_fix!()