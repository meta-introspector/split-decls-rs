macro_rules! Parenthesized {
    () => {
        # [doc = " Parenthesis helper"] pub (crate) struct Parenthesized < T > (pub (crate) T) ;
    };
}

Parenthesized!()