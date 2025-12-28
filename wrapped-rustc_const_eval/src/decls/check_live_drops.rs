macro_rules! deps {
    () => {
        ConstCx!();
        CheckLiveDrops!();
        Checker!();
    };
}

macro_rules! check_live_drops {
    () => {
        deps!();
        # [doc = " Look for live drops in a const context."] # [doc = ""] # [doc = " This is separate from the rest of the const checking logic because it must run after drop"] # [doc = " elaboration."] pub fn check_live_drops < 'tcx > (tcx : TyCtxt < 'tcx > , body : & mir :: Body < 'tcx >) { let ccx = ConstCx :: new (tcx , body) ; if ccx . const_kind . is_none () { return ; } if tcx . has_attr (body . source . def_id () , sym :: rustc_do_not_const_check) { return ; } if ! checking_enabled (& ccx) { return ; } let mut visitor = CheckLiveDrops { checker : Checker :: new (& ccx) } ; visitor . visit_body (body) ; }
    };
}

check_live_drops!();