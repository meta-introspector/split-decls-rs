macro_rules! deps {
    () => {
        PrettyFormatter!();
        Result!();
        Formatter!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < 'a > Formatter for PrettyFormatter < 'a > { # [inline] fn begin_array < W > (& mut self , writer : & mut W) -> io :: Result < () > where W : ? Sized + io :: Write , { self . current_indent += 1 ; self . has_value = false ; writer . write_all (b"[") } # [inline] fn end_array < W > (& mut self , writer : & mut W) -> io :: Result < () > where W : ? Sized + io :: Write , { self . current_indent -= 1 ; if self . has_value { tri ! (writer . write_all (b"\n")) ; tri ! (indent (writer , self . current_indent , self . indent)) ; } writer . write_all (b"]") } # [inline] fn begin_array_value < W > (& mut self , writer : & mut W , first : bool) -> io :: Result < () > where W : ? Sized + io :: Write , { tri ! (writer . write_all (if first { b"\n" } else { b",\n" })) ; indent (writer , self . current_indent , self . indent) } # [inline] fn end_array_value < W > (& mut self , _writer : & mut W) -> io :: Result < () > where W : ? Sized + io :: Write , { self . has_value = true ; Ok (()) } # [inline] fn begin_object < W > (& mut self , writer : & mut W) -> io :: Result < () > where W : ? Sized + io :: Write , { self . current_indent += 1 ; self . has_value = false ; writer . write_all (b"{") } # [inline] fn end_object < W > (& mut self , writer : & mut W) -> io :: Result < () > where W : ? Sized + io :: Write , { self . current_indent -= 1 ; if self . has_value { tri ! (writer . write_all (b"\n")) ; tri ! (indent (writer , self . current_indent , self . indent)) ; } writer . write_all (b"}") } # [inline] fn begin_object_key < W > (& mut self , writer : & mut W , first : bool) -> io :: Result < () > where W : ? Sized + io :: Write , { tri ! (writer . write_all (if first { b"\n" } else { b",\n" })) ; indent (writer , self . current_indent , self . indent) } # [inline] fn begin_object_value < W > (& mut self , writer : & mut W) -> io :: Result < () > where W : ? Sized + io :: Write , { writer . write_all (b": ") } # [inline] fn end_object_value < W > (& mut self , _writer : & mut W) -> io :: Result < () > where W : ? Sized + io :: Write , { self . has_value = true ; Ok (()) } }
    };
}

impl_168!();