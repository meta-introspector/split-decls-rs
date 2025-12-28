macro_rules! StrStep {
    () => {
        # [derive (Debug)] pub enum StrStep < 'a > { Token { kind : SyntaxKind , text : & 'a str } , Enter { kind : SyntaxKind } , Exit , Error { msg : & 'a str , pos : usize } , }
    };
}

StrStep!();