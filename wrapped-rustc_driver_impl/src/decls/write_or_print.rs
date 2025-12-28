macro_rules! write_or_print {
    () => {
        fn write_or_print (out : & str , sess : & Session) { sess . io . output_file . as_ref () . unwrap_or (& OutFileName :: Stdout) . overwrite (out , sess) ; }
    };
}

write_or_print!();