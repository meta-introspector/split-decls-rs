macro_rules! Conflict {
    () => {
        # [doc = " Describes a conflicting entry as comparison between 'our' version and 'their' version of it."] # [doc = ""] # [doc = " If one side isn't specified, it is assumed to have modified the entry. In general, there would be no conflict"] # [doc = " if both parties ended up in the same state."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Debug)] pub enum Conflict { # [doc = " Both deleted a different version of the entry."] BothDeleted , # [doc = " We added, they modified, ending up in different states."] AddedByUs , # [doc = " They deleted the entry, we modified it."] DeletedByThem , # [doc = " They added the entry, we modified it, ending up in different states."] AddedByThem , # [doc = " We deleted the entry, they modified it, ending up in different states."] DeletedByUs , # [doc = " Both added the entry in different states."] BothAdded , # [doc = " Both modified the entry, ending up in different states."] BothModified , }
    };
}

Conflict!();