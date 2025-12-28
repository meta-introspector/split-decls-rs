macro_rules! Delay {
    () => {
        # [doc = " What to do if delay is supported by a process filter."] # [derive (Default , Debug , Copy , Clone)] pub enum Delay { # [doc = " Use delayed processing for this entry."] # [doc = ""] # [doc = " Note that it's up to the filter to determine whether or not the processing should be delayed."] # [default] Allow , # [doc = " Do not delay the processing, and force it to happen immediately. In this case, no delayed processing will occur"] # [doc = " even if the filter supports it."] # [doc = ""] # [doc = " This is the default as it requires no special precautions to be taken by the caller as"] # [doc = " outputs will be produced immediately."] Forbid , }
    };
}

Delay!()