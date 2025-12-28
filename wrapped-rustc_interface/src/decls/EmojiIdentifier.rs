macro_rules! EmojiIdentifier {
    () => {
        # [derive (Diagnostic)] # [diag (interface_emoji_identifier)] pub struct EmojiIdentifier { # [primary_span] pub spans : Vec < Span > , pub ident : Symbol , }
    };
}

EmojiIdentifier!()