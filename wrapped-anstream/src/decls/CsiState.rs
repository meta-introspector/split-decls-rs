macro_rules! CsiState {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum CsiState { Normal , PrepareCustomColor , Ansi256 , Rgb , Underline , }
    };
}

CsiState!();