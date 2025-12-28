macro_rules! TitleStyle {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq)] enum TitleStyle { MainHeader , Header , Secondary , }
    };
}

TitleStyle!();