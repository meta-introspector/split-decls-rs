// Generated macro for setup_line_renderer_range (function)
macro_rules! Depcrate_sharedsetup_line_renderer_range {
() => {
// Module: crate::shared
// Provides: {"setup_line_renderer_range"}
// Dependencies: {}
# [allow (unused)] # [cfg (feature = "prodash-render-line")] pub fn setup_line_renderer_range (progress : & std :: sync :: Arc < prodash :: tree :: Root > , levels : std :: ops :: RangeInclusive < prodash :: progress :: key :: Level > ,) -> prodash :: render :: line :: JoinHandle { prodash :: render :: line (std :: io :: stderr () , std :: sync :: Arc :: downgrade (progress) , prodash :: render :: line :: Options { level_filter : Some (levels) , frames_per_second : DEFAULT_FRAME_RATE , initial_delay : Some (std :: time :: Duration :: from_millis (1000)) , timestamp : true , throughput : true , hide_cursor : true , .. prodash :: render :: line :: Options :: default () } . auto_configure (prodash :: render :: line :: StreamKind :: Stderr) ,) }
};
}
