macro_rules! write_msg_text {
    () => {
        fn write_msg_text (out : & mut dyn Write , msg : & str) -> io :: Result < () > { log :: debug ! ("> {msg}") ; write ! (out , "Content-Length: {}\r\n\r\n" , msg . len ()) ? ; out . write_all (msg . as_bytes ()) ? ; out . flush () ? ; Ok (()) }
    };
}

write_msg_text!();