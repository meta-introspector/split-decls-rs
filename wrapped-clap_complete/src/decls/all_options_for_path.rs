macro_rules! all_options_for_path {
    () => {
        fn all_options_for_path (cmd : & Command , path : & str) -> String { debug ! ("all_options_for_path: path={path}") ; let p = utils :: find_subcommand_with_path (cmd , path . split ("__") . skip (1) . collect ()) ; let mut opts = String :: new () ; for short in utils :: shorts_and_visible_aliases (p) { write ! (& mut opts , "-{short} ") . expect ("writing to String is infallible") ; } for long in utils :: longs_and_visible_aliases (p) { write ! (& mut opts , "--{long} ") . expect ("writing to String is infallible") ; } for pos in p . get_positionals () { if let Some (vals) = utils :: possible_values (pos) { for value in vals { write ! (& mut opts , "{} " , value . get_name ()) . expect ("writing to String is infallible") ; } } else { write ! (& mut opts , "{pos} ") . expect ("writing to String is infallible") ; } } for (sc , _) in utils :: subcommands (p) { write ! (& mut opts , "{sc} ") . expect ("writing to String is infallible") ; } opts . pop () ; opts }
    };
}

all_options_for_path!();