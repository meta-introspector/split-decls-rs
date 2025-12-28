macro_rules! only_last_segment {
    () => {
        fn only_last_segment (mut ty : & Type) -> Option < & PathSegment > { while let Type :: Group (syn :: TypeGroup { elem , .. }) = ty { ty = elem ; } match ty { Type :: Path (TypePath { qself : None , path : Path { leading_colon : None , segments , } , }) => only_one (segments . iter ()) , _ => None , } }
    };
}

only_last_segment!()