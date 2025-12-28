macro_rules! deps {
    () => {
        Generator!();
        ContextStack!();
    };
}

macro_rules! yield_from {
    () => {
        deps!();
        # [doc = " `yield_from`"] # [deprecated (since = "0.6.18" , note = "please use `scope` version instead")] pub fn yield_from < A : Any , T : Any > (mut g : Generator < A , T >) -> Option < A > { let env = ContextStack :: current () ; let context = env . top () ; let mut p = context . get_para () ; while unlikely (! g . is_done ()) { match g . raw_send (p) { None => return None , Some (r) => raw_yield (& env , context , r) , } p = context . get_para () ; } drop (g) ; p }
    };
}

yield_from!();