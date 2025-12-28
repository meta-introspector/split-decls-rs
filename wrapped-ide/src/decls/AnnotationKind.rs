macro_rules! deps {
    () => {
        Runnable!();
        NavigationTarget!();
    };
}

macro_rules! AnnotationKind {
    () => {
        deps!();
        # [derive (Debug , Hash , PartialEq , Eq)] pub enum AnnotationKind { Runnable (Runnable) , HasImpls { pos : FilePosition , data : Option < Vec < NavigationTarget > > } , HasReferences { pos : FilePosition , data : Option < Vec < FileRange > > } , }
    };
}

AnnotationKind!()