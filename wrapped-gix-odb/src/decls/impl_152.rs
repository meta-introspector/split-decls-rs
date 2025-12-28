macro_rules! deps {
    () => {
        Proxy!();
        Storage!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        # [doc = " Memory Access"] impl < T > Proxy < T > { # [doc = " Take all the objects in memory so far, with the memory storage itself and return it."] # [doc = ""] # [doc = " The instance will remain in a state where it won't be able to store objects in memory at all,"] # [doc = " they will now be stored in the underlying object database."] # [doc = " This mode makes the proxy fully transparent."] # [doc = ""] # [doc = " To avoid that, use [`reset_object_memory()`](Self::reset_object_memory()) or return the storage"] # [doc = " using [`set_object_memory()`](Self::set_object_memory())."] pub fn take_object_memory (& mut self) -> Option < Storage > { self . memory . take () . map (RefCell :: into_inner) } # [doc = " Set the object storage to contain only `new` objects, and return whichever objects were there previously."] pub fn set_object_memory (& mut self , new : Storage) -> Option < Storage > { let previous = self . take_object_memory () ; self . memory = Some (RefCell :: new (new)) ; previous } # [doc = " If objects aren't written to memory yet, this will happen after the call."] # [doc = ""] # [doc = " Otherwise, no change will be performed."] pub fn enable_object_memory (& mut self) -> & mut Self { if self . memory . is_none () { self . memory = Some (Default :: default ()) ; } self } # [doc = " Reset the internal storage to be empty, and return the previous storage, with all objects"] # [doc = " it contained."] # [doc = ""] # [doc = " Note that this does nothing if this instance didn't contain object memory in the first place."] # [doc = " In that case, set it explicitly."] pub fn reset_object_memory (& self) -> Option < Storage > { self . memory . as_ref () . map (| m | std :: mem :: take (& mut * m . borrow_mut ())) } # [doc = " Return the amount of objects currently stored in memory."] pub fn num_objects_in_memory (& self) -> usize { self . memory . as_ref () . map_or (0 , | m | m . borrow () . len ()) } }
    };
}

impl_152!();