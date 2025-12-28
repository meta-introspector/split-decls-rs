macro_rules! deps {
    () => {
        Push!();
        Fetch!();
    };
}

macro_rules! Instruction {
    () => {
        deps!();
        # [doc = " Tells what to do and is derived from a [`RefSpec`][crate::RefSpecRef]."] # [derive (PartialOrd , Ord , PartialEq , Eq , Copy , Clone , Hash , Debug)] pub enum Instruction < 'a > { # [doc = " An instruction for pushing."] Push (instruction :: Push < 'a >) , # [doc = " An instruction for fetching."] Fetch (instruction :: Fetch < 'a >) , }
    };
}

Instruction!();