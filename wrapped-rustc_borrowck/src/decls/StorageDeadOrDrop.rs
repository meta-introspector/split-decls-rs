macro_rules! StorageDeadOrDrop {
    () => {
        # [doc = " Which case a StorageDeadOrDrop is for."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] enum StorageDeadOrDrop < 'tcx > { LocalStorageDead , BoxedStorageDead , Destructor (Ty < 'tcx >) , }
    };
}

StorageDeadOrDrop!();