// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_query_system/src/query/plumbing.rs
// Error: expected square brackets
// Problematic line: line 27

use crate::query::job::{QueryInfo, QueryJob, QueryJobId, QueryJobInfo, QueryLatch, report_cycle};
use crate::query::{QueryContext, QueryMap, QueryStackFrame, SerializedDepNodeIndex};

#[inline]
fn equivalent_key<K: Eq, V>(k: &K) -> impl Fn(&(K, V)) -> bool + '_ {
    move |x| x.0 == *k
}
