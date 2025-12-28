macro_rules! UsedParser {
    () => {
        # [derive (Default)] pub (crate) struct UsedParser { first_compiler : Option < Span > , first_linker : Option < Span > , }
    };
}

UsedParser!();