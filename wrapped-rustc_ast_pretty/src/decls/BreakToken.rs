macro_rules! BreakToken {
    () => {
        # [derive (Clone , Copy , Default , PartialEq)] pub (crate) struct BreakToken { offset : isize , blank_space : isize , pre_break : Option < char > , }
    };
}

BreakToken!()