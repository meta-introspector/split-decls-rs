// Generated macro for QlogLevel (enum)
macro_rules! DepcrateQlogLevel {
() => {
// Module: crate
// Provides: {"QlogLevel"}
// Dependencies: {}
# [doc = " Qlog logging level."] # [repr (C)] # [cfg (feature = "qlog")] # [cfg_attr (docsrs , doc (cfg (feature = "qlog")))] pub enum QlogLevel { # [doc = " Logs any events of Core importance."] Core = 0 , # [doc = " Logs any events of Core and Base importance."] Base = 1 , # [doc = " Logs any events of Core, Base and Extra importance"] Extra = 2 , }
};
}
