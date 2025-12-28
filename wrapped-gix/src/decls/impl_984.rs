macro_rules! deps {
    () => {
        Error!();
        SchemePermission!();
        Init!();
        Allow!();
        Protocol!();
        User!();
        Default!();
    };
}

macro_rules! impl_984 {
    () => {
        deps!();
        # [doc = " Init"] impl SchemePermission { # [doc = " NOTE: _intentionally without leniency_"] pub fn from_config (config : & gix_config :: File < 'static > , mut filter : fn (& gix_config :: file :: Metadata) -> bool ,) -> Result < Self , config :: protocol :: allow :: Error > { let allow : Option < Allow > = config . string_filter ("protocol.allow" , & mut filter) . map (| value | Protocol :: ALLOW . try_into_allow (value , None)) . transpose () ? ; let mut saw_user = allow == Some (Allow :: User) ; let allow_per_scheme = match config . sections_by_name_and_filter ("protocol" , & mut filter) { Some (it) => { let mut map = BTreeMap :: default () ; for (section , scheme) in it . filter_map (| section | { section . header () . subsection_name () . and_then (| scheme | { scheme . to_str () . ok () . map (| scheme | (section , gix_url :: Scheme :: from (scheme))) }) }) { if let Some (value) = section . value ("allow") . map (| value | Protocol :: ALLOW . try_into_allow (value , Some (scheme . as_str ()))) . transpose () ? { saw_user |= value == Allow :: User ; map . insert (scheme , value) ; } } map } None => Default :: default () , } ; let user_allowed = saw_user . then (| | { config . string_filter (gitoxide :: Allow :: PROTOCOL_FROM_USER , & mut filter) . is_none_or (| val | val . as_ref () == "1") }) ; Ok (SchemePermission { allow , allow_per_scheme , user_allowed , }) } }
    };
}

impl_984!();