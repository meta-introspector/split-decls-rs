macro_rules! deps {
    () => {
        MappedLocalTime!();
    };
}

macro_rules! impl_674 {
    () => {
        deps!();
        impl < T > MappedLocalTime < T > { # [doc = " Returns `Some` if the time zone mapping has a single result."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns `None` if local time falls in a _fold_ or _gap_ in the local time, or if there was"] # [doc = " an error."] # [must_use] pub fn single (self) -> Option < T > { match self { MappedLocalTime :: Single (t) => Some (t) , _ => None , } } # [doc = " Returns the earliest possible result of the time zone mapping."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns `None` if local time falls in a _gap_ in the local time, or if there was an error."] # [must_use] pub fn earliest (self) -> Option < T > { match self { MappedLocalTime :: Single (t) | MappedLocalTime :: Ambiguous (t , _) => Some (t) , _ => None , } } # [doc = " Returns the latest possible result of the time zone mapping."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns `None` if local time falls in a _gap_ in the local time, or if there was an error."] # [must_use] pub fn latest (self) -> Option < T > { match self { MappedLocalTime :: Single (t) | MappedLocalTime :: Ambiguous (_ , t) => Some (t) , _ => None , } } # [doc = " Maps a `MappedLocalTime<T>` into `MappedLocalTime<U>` with given function."] # [must_use] pub fn map < U , F : FnMut (T) -> U > (self , mut f : F) -> MappedLocalTime < U > { match self { MappedLocalTime :: None => MappedLocalTime :: None , MappedLocalTime :: Single (v) => MappedLocalTime :: Single (f (v)) , MappedLocalTime :: Ambiguous (min , max) => MappedLocalTime :: Ambiguous (f (min) , f (max)) , } } # [doc = " Maps a `MappedLocalTime<T>` into `MappedLocalTime<U>` with given function."] # [doc = ""] # [doc = " Returns `MappedLocalTime::None` if the function returns `None`."] # [must_use] pub (crate) fn and_then < U , F : FnMut (T) -> Option < U > > (self , mut f : F) -> MappedLocalTime < U > { match self { MappedLocalTime :: None => MappedLocalTime :: None , MappedLocalTime :: Single (v) => match f (v) { Some (new) => MappedLocalTime :: Single (new) , None => MappedLocalTime :: None , } , MappedLocalTime :: Ambiguous (min , max) => match (f (min) , f (max)) { (Some (min) , Some (max)) => MappedLocalTime :: Ambiguous (min , max) , _ => MappedLocalTime :: None , } , } } }
    };
}

impl_674!()