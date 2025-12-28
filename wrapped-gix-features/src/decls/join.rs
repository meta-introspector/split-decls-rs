macro_rules! join {
    () => {
        # [doc = " Runs `left` and `right` in parallel, returning their output when both are done."] pub fn join < O1 : Send , O2 : Send > (left : impl FnOnce () -> O1 + Send , right : impl FnOnce () -> O2 + Send) -> (O1 , O2) { std :: thread :: scope (| s | { let left = std :: thread :: Builder :: new () . name ("gitoxide.join.left" . into ()) . spawn_scoped (s , left) . expect ("valid name") ; let right = std :: thread :: Builder :: new () . name ("gitoxide.join.right" . into ()) . spawn_scoped (s , right) . expect ("valid name") ; (left . join () . unwrap () , right . join () . unwrap ()) }) }
    };
}

join!();