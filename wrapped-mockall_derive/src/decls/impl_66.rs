macro_rules! deps {
    () => {
        AttrFormatter!();
        Methods!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl Methods { # [doc = " Are all of these methods static?"] fn all_static (& self) -> bool { self . 0 . iter () . all (| meth | meth . is_static ()) } fn checkpoints (& self) -> Vec < impl ToTokens > { self . 0 . iter () . filter (| meth | ! meth . is_static ()) . map (| meth | meth . checkpoint ()) . collect :: < Vec < _ > > () } # [doc = " Return a fragment of code to initialize struct fields during default()"] fn default_inits (& self) -> Vec < TokenStream > { self . 0 . iter () . filter (| meth | ! meth . is_static ()) . map (| meth | { let name = meth . name () ; let attrs = AttrFormatter :: new (& meth . attrs) . doc (false) . format () ; quote ! (# (# attrs) * # name : Default :: default ()) }) . collect :: < Vec < _ > > () } fn field_definitions (& self , modname : & Ident) -> Vec < TokenStream > { self . 0 . iter () . filter (| meth | ! meth . is_static ()) . map (| meth | meth . field_definition (Some (modname))) . collect :: < Vec < _ > > () } fn priv_mods (& self) -> Vec < impl ToTokens > { self . 0 . iter () . map (| meth | meth . priv_module ()) . collect :: < Vec < _ > > () } }
    };
}

impl_66!()