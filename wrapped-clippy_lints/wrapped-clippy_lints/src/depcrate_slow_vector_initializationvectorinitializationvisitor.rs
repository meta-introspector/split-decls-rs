// Generated macro for VectorInitializationVisitor (struct)
macro_rules! Depcrate_slow_vector_initializationVectorInitializationVisitor {
() => {
// Module: crate::slow_vector_initialization
// Provides: {"VectorInitializationVisitor"}
// Dependencies: {}
# [doc = " `VectorInitializationVisitor` searches for unsafe or slow vector initializations for the given"] # [doc = " vector."] struct VectorInitializationVisitor < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , # [doc = " Contains the information."] vec_alloc : VecAllocation < 'tcx > , # [doc = " Contains the slow initialization expression, if one was found."] slow_expression : Option < InitializationType < 'tcx > > , # [doc = " `true` if the initialization of the vector has been found on the visited block."] initialization_found : bool , }
};
}
