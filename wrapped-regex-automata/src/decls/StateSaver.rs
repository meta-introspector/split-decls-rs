macro_rules! deps {
    () => {
        DFA!();
        State!();
        LazyStateID!();
    };
}

macro_rules! StateSaver {
    () => {
        deps!();
        # [doc = " A simple type that encapsulates the saving of a state ID through a cache"] # [doc = " clearing."] # [doc = ""] # [doc = " A state ID can be marked for saving with ToSave, while a state ID can be"] # [doc = " saved itself with Saved."] # [derive (Clone , Debug)] enum StateSaver { # [doc = " An empty state saver. In this case, no states (other than the special"] # [doc = " sentinel states) are preserved after clearing the cache."] None , # [doc = " An ID of a state (and the state itself) that should be preserved after"] # [doc = " the lazy DFA's cache has been cleared. After clearing, the updated ID"] # [doc = " is stored in 'Saved' since it may have changed."] ToSave { id : LazyStateID , state : State } , # [doc = " An ID that of a state that has been persisted through a lazy DFA"] # [doc = " cache clearing. The ID recorded here corresponds to an ID that was"] # [doc = " once marked as ToSave. The IDs are likely not equivalent even though"] # [doc = " the states they point to are."] Saved (LazyStateID) , }
    };
}

StateSaver!();