// Generated macro for Summary (struct)
macro_rules! Depcrate_statsSummary {
() => {
// Module: crate::stats
// Provides: {"Summary"}
// Dependencies: {}
# [doc = " Summary is the summary of a query statistics."] # [derive (Clone , PartialEq , :: prost :: Message)] pub struct Summary { # [doc = " Total bytes processed per second."] # [prost (int64 , tag = "1")] pub bytes_processed_per_second : i64 , # [doc = " Total lines processed per second."] # [prost (int64 , tag = "2")] pub lines_processed_per_second : i64 , # [doc = " Total bytes processed."] # [prost (int64 , tag = "3")] pub total_bytes_processed : i64 , # [doc = " Total lines processed."] # [prost (int64 , tag = "4")] pub total_lines_processed : i64 , # [doc = " Execution time in seconds."] # [doc = " In addition to internal calculations this is also returned by the HTTP API."] # [doc = " Grafana expects time values to be returned in seconds as float."] # [prost (double , tag = "5")] pub exec_time : f64 , # [doc = " Queue time in seconds."] # [doc = " In addition to internal calculations this is also returned by the HTTP API."] # [doc = " Grafana expects time values to be returned in seconds as float."] # [prost (double , tag = "6")] pub queue_time : f64 , # [doc = " Total of subqueries created to fulfill this query."] # [prost (int64 , tag = "7")] pub subqueries : i64 , }
};
}
