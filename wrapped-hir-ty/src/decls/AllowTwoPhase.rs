macro_rules! AllowTwoPhase {
    () => {
        # [doc = " At least for initial deployment, we want to limit two-phase borrows to"] # [doc = " only a few specific cases. Right now, those are mostly \"things that desugar\""] # [doc = " into method calls:"] # [doc = " - using `x.some_method()` syntax, where some_method takes `&mut self`,"] # [doc = " - using `Foo::some_method(&mut x, ...)` syntax,"] # [doc = " - binary assignment operators (`+=`, `-=`, `*=`, etc.)."] # [doc = ""] # [doc = " Anything else should be rejected until generalized two-phase borrow support"] # [doc = " is implemented. Right now, dataflow can't handle the general case where there"] # [doc = " is more than one use of a mutable borrow, and we don't want to accept too much"] # [doc = " new code via two-phase borrows, so we try to limit where we create two-phase"] # [doc = " capable mutable borrows."] # [doc = " See #49434 for tracking."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum AllowTwoPhase { Yes , No , }
    };
}

AllowTwoPhase!()