macro_rules! Effects {
    () => {
        # [doc = " A set of text effects"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;"] # [doc = " ```"] # [derive (Copy , Clone , Default , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Effects (u16) ;
    };
}

Effects!()