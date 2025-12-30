// Generated macro for write_timer_to_summary (function)
macro_rules! Depcrate_utilswrite_timer_to_summary {
() => {
// Module: crate::utils
// Provides: {"write_timer_to_summary"}
// Dependencies: {}
# [doc = " Write the formatted statistics of the timer to a Github Actions summary."] pub fn write_timer_to_summary (path : & str , timer : & Timer) -> anyhow :: Result < () > { use std :: io :: Write ; let mut file = std :: fs :: File :: options () . append (true) . create (true) . open (path) ? ; writeln ! (file , r#"# Step durations

```
{}
```
"# , timer . format_stats ()) ? ; Ok (()) }
};
}
