macro_rules! deps {
    () => {
        Bash!();
        Fish!();
        Shell!();
        Zsh!();
        Elvish!();
        PowerShell!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl ValueEnum for Shell { fn value_variants < 'a > () -> & 'a [Self] { & [Shell :: Bash , Shell :: Elvish , Shell :: Fish , Shell :: PowerShell , Shell :: Zsh ,] } fn to_possible_value (& self) -> Option < PossibleValue > { Some (match self { Shell :: Bash => PossibleValue :: new ("bash") , Shell :: Elvish => PossibleValue :: new ("elvish") , Shell :: Fish => PossibleValue :: new ("fish") , Shell :: PowerShell => PossibleValue :: new ("powershell") , Shell :: Zsh => PossibleValue :: new ("zsh") , }) } }
    };
}

impl_51!()