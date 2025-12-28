macro_rules! deps {
    () => {
        PreUpdateOldValueAccessor!();
        PreUpdateNewValueAccessor!();
    };
}

macro_rules! PreUpdateCase {
    () => {
        deps!();
        # [doc = " The possible cases for when a PreUpdateHook gets triggered. Allows access to the relevant"] # [doc = " functions for each case through the contained values."] # [derive (Debug)] pub enum PreUpdateCase { # [doc = " Pre-update hook was triggered by an insert."] Insert (PreUpdateNewValueAccessor) , # [doc = " Pre-update hook was triggered by a delete."] Delete (PreUpdateOldValueAccessor) , # [doc = " Pre-update hook was triggered by an update."] Update { # [allow (missing_docs)] old_value_accessor : PreUpdateOldValueAccessor , # [allow (missing_docs)] new_value_accessor : PreUpdateNewValueAccessor , } , # [doc = " This variant is not normally produced by SQLite. You may encounter it"] # [doc = " if you're using a different version than what's supported by this library."] Unknown , }
    };
}

PreUpdateCase!()