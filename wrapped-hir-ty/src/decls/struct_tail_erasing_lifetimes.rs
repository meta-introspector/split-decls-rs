macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! struct_tail_erasing_lifetimes {
    () => {
        deps!();
        fn struct_tail_erasing_lifetimes < 'a > (db : & 'a dyn HirDatabase , pointee : Ty < 'a >) -> Ty < 'a > { match pointee . kind () { TyKind :: Adt (def , args) => { let struct_id = match def . inner () . id { AdtId :: StructId (id) => id , _ => return pointee , } ; let data = struct_id . fields (db) ; let mut it = data . fields () . iter () . rev () ; match it . next () { Some ((f , _)) => { let last_field_ty = field_ty (db , struct_id . into () , f , & args) ; struct_tail_erasing_lifetimes (db , last_field_ty) } None => pointee , } } TyKind :: Tuple (tys) => { if let Some (last_field_ty) = tys . iter () . next_back () { struct_tail_erasing_lifetimes (db , last_field_ty) } else { pointee } } _ => pointee , } }
    };
}

struct_tail_erasing_lifetimes!();