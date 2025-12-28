macro_rules! deps {
    () => {
        Any!();
        Validate!();
        Link!();
        Note!();
        Key!();
        SubSectionRequirement!();
    };
}

macro_rules! impl_760 {
    () => {
        deps!();
        # [doc = " Builder"] impl < T : Validate > Any < T > { # [doc = " Set the subsection requirement to non-default values."] pub const fn with_subsection_requirement (mut self , requirement : Option < SubSectionRequirement >) -> Self { self . subsection_requirement = requirement ; self } # [doc = " Associate an environment variable with this key."] # [doc = ""] # [doc = " This is mainly useful for enriching error messages."] pub const fn with_environment_override (mut self , var : & 'static str) -> Self { self . link = Some (Link :: EnvironmentOverride (var)) ; self } # [doc = " Set a link to another key which serves as fallback to provide a value if this key is not set."] pub const fn with_fallback (mut self , key : & 'static dyn Key) -> Self { self . link = Some (Link :: FallbackKey (key)) ; self } # [doc = " Attach an informative message to this key."] pub const fn with_note (mut self , message : & 'static str) -> Self { self . note = Some (Note :: Informative (message)) ; self } # [doc = " Inform about a deviation in how this key is interpreted."] pub const fn with_deviation (mut self , message : & 'static str) -> Self { self . note = Some (Note :: Deviation (message)) ; self } }
    };
}

impl_760!();