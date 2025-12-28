macro_rules! ProjectionElem {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum ProjectionElem < V , T > { Deref , Field (Either < FieldId , TupleFieldId >) , ClosureField (usize) , Index (V) , ConstantIndex { offset : u64 , from_end : bool } , Subslice { from : u64 , to : u64 } , OpaqueCast (T) , }
    };
}

ProjectionElem!()