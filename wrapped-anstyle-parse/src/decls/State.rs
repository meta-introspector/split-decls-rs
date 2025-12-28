macro_rules! State {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [repr (u8)] # [derive (Default)] pub enum State { Anywhere = 0 , CsiEntry = 1 , CsiIgnore = 2 , CsiIntermediate = 3 , CsiParam = 4 , DcsEntry = 5 , DcsIgnore = 6 , DcsIntermediate = 7 , DcsParam = 8 , DcsPassthrough = 9 , Escape = 10 , EscapeIntermediate = 11 , # [default] Ground = 12 , OscString = 13 , SosPmApcString = 14 , Utf8 = 15 , }
    };
}

State!();