macro_rules! deps {
    () => {
        Conflict!();
    };
}

macro_rules! Outcome {
    () => {
        deps!();
        # [doc = " The outcome produced by [`tree()`](crate::tree())."] # [derive (Clone)] pub struct Outcome < 'a > { # [doc = " The ready-made (but unwritten) *base* tree, including all non-conflicting changes, and the changes that had"] # [doc = " conflicts which could be resolved automatically."] # [doc = ""] # [doc = " This means, if all of their changes were conflicting, this will be equivalent to the *base* tree."] pub tree : gix_object :: tree :: Editor < 'a > , # [doc = " The set of conflicts we encountered. Can be empty to indicate there was no conflict."] # [doc = " Note that conflicts might have been auto-resolved, but they are listed here for completeness."] # [doc = " Use [`has_unresolved_conflicts()`](Outcome::has_unresolved_conflicts()) to see if any action is needed"] # [doc = " before using [`tree`](Outcome::tree)."] pub conflicts : Vec < Conflict > , # [doc = " `true` if `conflicts` contains only a single [*unresolved* conflict](ResolutionFailure) in the last slot, but"] # [doc = " possibly more [resolved ones](Resolution) before that."] # [doc = " This also makes this outcome a very partial merge that cannot be completed."] # [doc = " Only set if [`fail_on_conflict`](Options::fail_on_conflict) is `true`."] pub failed_on_first_unresolved_conflict : bool , }
    };
}

Outcome!();