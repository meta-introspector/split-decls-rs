macro_rules! skip_branch {
    () => {
        # [doc = " Tells loom to stop exploring possible concurrent execution starting at this"] # [doc = " point."] # [doc = ""] # [doc = " Unlike `stop_exploring`, exploration cannot be restarted by `explore`."] pub fn skip_branch () { execution (| execution | execution . path . skip_branch ()) }
    };
}

skip_branch!()