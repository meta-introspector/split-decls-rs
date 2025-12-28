macro_rules! deps {
    () => {
        DropData!();
        TypeChecker!();
        LocalUseMap!();
    };
}

macro_rules! LivenessContext {
    () => {
        deps!();
        # [doc = " Contextual state for the type-liveness coroutine."] struct LivenessContext < 'a , 'typeck , 'tcx > { # [doc = " Current type-checker, giving us our inference context etc."] # [doc = ""] # [doc = " This also stores the body we're currently analyzing."] typeck : & 'a mut TypeChecker < 'typeck , 'tcx > , # [doc = " Defines the `PointIndex` mapping"] location_map : & 'a DenseLocationMap , # [doc = " Mapping to/from the various indices used for initialization tracking."] move_data : & 'a MoveData < 'tcx > , # [doc = " Cache for the results of `dropck_outlives` query."] drop_data : FxIndexMap < Ty < 'tcx > , DropData < 'tcx > > , # [doc = " Results of dataflow tracking which variables (and paths) have been"] # [doc = " initialized. Computed lazily when needed by drop-liveness."] flow_inits : Option < ResultsCursor < 'a , 'tcx , MaybeInitializedPlaces < 'a , 'tcx > > > , # [doc = " Index indicating where each variable is assigned, used, or"] # [doc = " dropped."] local_use_map : & 'a LocalUseMap , }
    };
}

LivenessContext!();