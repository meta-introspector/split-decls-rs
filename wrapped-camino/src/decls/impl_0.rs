macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_0 {
    () => {
        deps!();
        # [doc = " The [`Arbitrary`] impl for `Utf8PathBuf` returns a path with between 0 and 8 components,"] # [doc = " joined by the [`MAIN_SEPARATOR`](std::path::MAIN_SEPARATOR) for the platform. (Each component is"] # [doc = " randomly generated, and may itself contain one or more separators.)"] # [doc = ""] # [doc = " On Unix, this generates an absolute path half of the time and a relative path the other half."] # [doc = ""] # [doc = " On Windows, this implementation doesn't currently generate"] # [doc = " [`Utf8PrefixComponent`](crate::Utf8PrefixComponent) instances, though in the future it might."] # [cfg (feature = "proptest1")] impl Arbitrary for Utf8PathBuf { type Parameters = < String as Arbitrary > :: Parameters ; type Strategy = BoxedStrategy < Self > ; fn arbitrary_with (args : Self :: Parameters) -> Self :: Strategy { (any :: < bool > () , prop :: collection :: vec (any_with :: < String > (args) , 0 .. 8) ,) . prop_map (| (is_relative , components) | { let initial_component = if is_relative { Some (format ! ("{}" , std :: path :: MAIN_SEPARATOR)) } else { None } ; initial_component . into_iter () . chain (components) . collect () }) . boxed () } }
    };
}

impl_0!();