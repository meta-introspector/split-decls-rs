macro_rules! Executor01As03 {
    () => {
        # [doc = " Converts a futures 0.1 [`Executor`](futures_01::future::Executor) into a"] # [doc = " futures 0.3 [`Spawn`](futures_task::Spawn)."] # [derive (Debug , Clone)] pub struct Executor01As03 < Ex > { executor01 : Ex , }
    };
}

Executor01As03!();