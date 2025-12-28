macro_rules! deps {
    () => {
        PowerShell!();
        Bash!();
        Fish!();
        Elvish!();
        Zsh!();
    };
}

macro_rules! Shell {
    () => {
        deps!();
        # [doc = " Shell with auto-generated completion script available."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [non_exhaustive] pub enum Shell { # [doc = " Bourne Again `SHell` (bash)"] Bash , # [doc = " Elvish shell"] Elvish , # [doc = " Friendly Interactive `SHell` (fish)"] Fish , # [doc = " `PowerShell`"] PowerShell , # [doc = " Z `SHell` (zsh)"] Zsh , }
    };
}

Shell!();