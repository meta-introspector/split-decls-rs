macro_rules! deps {
    () => {
        Error!();
        Pending!();
        Upgraded!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        # [cfg (all (any (feature = "client" , feature = "server") , any (feature = "http1" , feature = "http2")))] impl Pending { pub (super) fn fulfill (self , upgraded : Upgraded) { trace ! ("pending upgrade fulfill") ; let _ = self . tx . send (Ok (upgraded)) ; } # [cfg (feature = "http1")] # [doc = " Don't fulfill the pending Upgrade, but instead signal that"] # [doc = " upgrades are handled manually."] pub (super) fn manual (self) { # [cfg (any (feature = "http1" , feature = "http2"))] trace ! ("pending upgrade handled manually") ; let _ = self . tx . send (Err (crate :: Error :: new_user_manual_upgrade ())) ; } }
    };
}

impl_221!();