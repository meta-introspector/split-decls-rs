macro_rules! PoloniusLocationTable {
    () => {
        # [doc = " Maps between a MIR Location, which identifies a particular"] # [doc = " statement within a basic block, to a \"rich location\", which"] # [doc = " identifies at a finer granularity. In particular, we distinguish"] # [doc = " the *start* of a statement and the *mid-point*. The mid-point is"] # [doc = " the point *just* before the statement takes effect; in particular,"] # [doc = " for an assignment `A = B`, it is the point where B is about to be"] # [doc = " written into A. This mid-point is a kind of hack to work around"] # [doc = " our inability to track the position information at sufficient"] # [doc = " granularity through outlives relations; however, the rich location"] # [doc = " table serves another purpose: it compresses locations from"] # [doc = " multiple words into a single u32."] pub struct PoloniusLocationTable { num_points : usize , statements_before_block : IndexVec < BasicBlock , usize > , }
    };
}

PoloniusLocationTable!()