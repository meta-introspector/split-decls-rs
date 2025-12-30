// Generated macro for BodyWithBorrowckFacts (struct)
macro_rules! Depcrate_consumersBodyWithBorrowckFacts {
() => {
// Module: crate::consumers
// Provides: {"BodyWithBorrowckFacts"}
// Dependencies: {}
# [doc = " A `Body` with information computed by the borrow checker. This struct is"] # [doc = " intended to be consumed by compiler consumers."] # [doc = ""] # [doc = " We need to include the MIR body here because the region identifiers must"] # [doc = " match the ones in the Polonius facts."] pub struct BodyWithBorrowckFacts < 'tcx > { # [doc = " A mir body that contains region identifiers."] pub body : Body < 'tcx > , # [doc = " The mir bodies of promoteds."] pub promoted : IndexVec < Promoted , Body < 'tcx > > , # [doc = " The set of borrows occurring in `body` with data about them."] pub borrow_set : BorrowSet < 'tcx > , # [doc = " Context generated during borrowck, intended to be passed to"] # [doc = " [`calculate_borrows_out_of_scope_at_location`]."] pub region_inference_context : RegionInferenceContext < 'tcx > , # [doc = " The table that maps Polonius points to locations in the table."] # [doc = " Populated when using [`ConsumerOptions::PoloniusInputFacts`]"] # [doc = " or [`ConsumerOptions::PoloniusOutputFacts`]."] pub location_table : Option < PoloniusLocationTable > , # [doc = " Polonius input facts."] # [doc = " Populated when using [`ConsumerOptions::PoloniusInputFacts`]"] # [doc = " or [`ConsumerOptions::PoloniusOutputFacts`]."] pub input_facts : Option < Box < PoloniusInput > > , # [doc = " Polonius output facts. Populated when using"] # [doc = " [`ConsumerOptions::PoloniusOutputFacts`]."] pub output_facts : Option < Box < PoloniusOutput > > , }
};
}
