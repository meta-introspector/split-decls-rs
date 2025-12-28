macro_rules! Step {
    () => {
        # [derive (Debug)] pub enum Step < 'a > { Token { kind : SyntaxKind , n_input_tokens : u8 } , FloatSplit { ends_in_dot : bool } , Enter { kind : SyntaxKind } , Exit , Error { msg : & 'a str } , }
    };
}

Step!()