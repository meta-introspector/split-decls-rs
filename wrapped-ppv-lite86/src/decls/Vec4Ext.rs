macro_rules! deps {
    () => {
        Vec4!();
    };
}

macro_rules! Vec4Ext {
    () => {
        deps!();
        # [doc = " Vec4 functions which may not be implemented yet for all Vec4 types."] # [doc = " NOTE: functions in this trait may be moved to Vec4 in any patch release. To avoid breakage,"] # [doc = " import Vec4Ext only together with Vec4, and don't qualify its methods."] pub trait Vec4Ext < W > { fn transpose4 (a : Self , b : Self , c : Self , d : Self) -> (Self , Self , Self , Self) where Self : Sized ; }
    };
}

Vec4Ext!();