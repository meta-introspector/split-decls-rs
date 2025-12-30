// Generated macro for impl_86 (impl)
macro_rules! Depcrate_aot_shells_shellimpl_86 {
() => {
// Module: crate::aot::shells::shell
// Provides: {"impl_86"}
// Dependencies: {}
impl ValueEnum for Shell { fn value_variants < 'a > () -> & 'a [Self] { & [Shell :: Bash , Shell :: Elvish , Shell :: Fish , Shell :: PowerShell , Shell :: Zsh ,] } fn to_possible_value (& self) -> Option < PossibleValue > { Some (match self { Shell :: Bash => PossibleValue :: new ("bash") , Shell :: Elvish => PossibleValue :: new ("elvish") , Shell :: Fish => PossibleValue :: new ("fish") , Shell :: PowerShell => PossibleValue :: new ("powershell") , Shell :: Zsh => PossibleValue :: new ("zsh") , }) } }
};
}
