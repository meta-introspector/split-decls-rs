macro_rules! deps {
    () => {
        Error!();
        Action!();
        Negotiate!();
        State!();
    };
}

macro_rules! impl_961 {
    () => {
        deps!();
        impl gix_protocol :: fetch :: Negotiate for Negotiate < '_ , '_ , '_ > { fn mark_complete_and_common_ref (& mut self) -> Result < negotiate :: Action , negotiate :: Error > { negotiate :: mark_complete_and_common_ref (& self . objects , self . refs , { let alternates = std :: mem :: take (& mut self . alternates) ; let open_options = self . open_options . clone () ; move | | -> Result < _ , std :: convert :: Infallible > { Ok (alternates . into_iter () . filter_map (move | path | { path . ancestors () . nth (1) . and_then (| git_dir | crate :: open_opts (git_dir , open_options . clone ()) . ok ()) }) . map (| repo | (repo . refs , repo . objects))) } } , self . negotiator . deref_mut () , & mut * self . graph , self . ref_map , self . shallow , negotiate :: make_refmapping_ignore_predicate (self . tags , self . ref_map) ,) } fn add_wants (& mut self , arguments : & mut Arguments , remote_ref_target_known : & [bool]) -> bool { negotiate :: add_wants (self . objects , arguments , self . ref_map , remote_ref_target_known , self . shallow , negotiate :: make_refmapping_ignore_predicate (self . tags , self . ref_map) ,) } fn one_round (& mut self , state : & mut negotiate :: one_round :: State , arguments : & mut Arguments , previous_response : Option < & gix_protocol :: fetch :: Response > ,) -> Result < (negotiate :: Round , bool) , negotiate :: Error > { negotiate :: one_round (self . negotiator . deref_mut () , & mut * self . graph , state , arguments , previous_response ,) } }
    };
}

impl_961!();