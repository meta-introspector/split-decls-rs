macro_rules! FluentNumberStyle {
    () => {
        # [derive (Clone , Copy , Debug , Default , Eq , Hash , PartialEq)] pub enum FluentNumberStyle { # [default] Decimal , Currency , Percent , }
    };
}

FluentNumberStyle!();