macro_rules! deps {
    () => {
        Sp!();
        ValueParser!();
        CasingStyle!();
        Method!();
        Deprecation!();
        Kind!();
        Action!();
        Name!();
    };
}

macro_rules! Item {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct Item { name : Name , casing : Sp < CasingStyle > , env_casing : Sp < CasingStyle > , ty : Option < Type > , doc_comment : Vec < Method > , methods : Vec < Method > , deprecations : Vec < Deprecation > , value_parser : Option < ValueParser > , action : Option < Action > , verbatim_doc_comment : bool , force_long_help : bool , next_display_order : Option < Method > , next_help_heading : Option < Method > , is_enum : bool , is_positional : bool , skip_group : bool , group_id : Name , group_methods : Vec < Method > , kind : Sp < Kind > , }
    };
}

Item!()