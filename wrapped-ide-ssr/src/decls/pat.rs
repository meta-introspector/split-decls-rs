macro_rules! pat {
    () => {
        pub (crate) fn pat (s : & str) -> Result < SyntaxNode , () > { fragment :: < ast :: Pat > ("const _: () = {let {} = ();};" , s) }
    };
}

pat!();