// Generated macro for impl_238 (impl)
macro_rules! Depcrate_options_file_nameimpl_238 {
() => {
// Module: crate::options::file_name
// Provides: {"impl_238"}
// Dependencies: {}
impl ShowIcons { pub fn deduce < V : Vars > (matches : & MatchedFlags < '_ > , vars : & V) -> Result < Self , OptionsError > { enum AlwaysOrAuto { Always , Automatic , } let force_icons = vars . get (vars :: EZA_ICONS_AUTO) . is_some () ; let mode_opt = matches . get (& flags :: ICONS) ? ; if ! force_icons && ! matches . has (& flags :: ICONS) ? && mode_opt . is_none () { return Ok (Self :: Never) ; } let mode = match mode_opt { Some (word) => match word . to_str () { Some ("always") => AlwaysOrAuto :: Always , Some ("auto" | "automatic") => AlwaysOrAuto :: Automatic , Some ("never") => return Ok (Self :: Never) , None => AlwaysOrAuto :: Automatic , _ => return Err (OptionsError :: BadArgument (& flags :: ICONS , word . into ())) , } , None => AlwaysOrAuto :: Automatic , } ; let width = if let Some (columns) = vars . get_with_fallback (vars :: EXA_ICON_SPACING , vars :: EZA_ICON_SPACING) . and_then (| s | s . into_string () . ok ()) { match columns . parse () { Ok (width) => width , Err (e) => { let source = NumberSource :: Env (vars . source (vars :: EXA_ICON_SPACING , vars :: EZA_ICON_SPACING) . unwrap () ,) ; return Err (OptionsError :: FailedParse (columns , source , e)) ; } } } else { 1 } ; match mode { AlwaysOrAuto :: Always => Ok (Self :: Always (width)) , AlwaysOrAuto :: Automatic => Ok (Self :: Automatic (width)) , } } }
};
}
