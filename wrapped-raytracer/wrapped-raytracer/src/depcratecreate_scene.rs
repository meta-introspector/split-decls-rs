// Generated macro for create_scene (function)
macro_rules! Depcratecreate_scene {
() => {
// Module: crate
// Provides: {"create_scene"}
// Dependencies: {}
# [doc = " Generate a Model containing a bunch of randomly placed spheres."] fn create_scene () -> Box < dyn Model > { let mut spheres : Vec < Sphere > = vec ! [Sphere { center : Vec3 (0.0 , 0.0 , - 1000.0) , radius : 1000.0 , material : Box :: new (Lambertian { albedo : Vec3 (1.0 , 0.6 , 0.5) , }) , } , Sphere { center : Vec3 (- 4.0 , 0.0 , 2.0) , radius : 2.0 , material : Box :: new (Lambertian { albedo : Vec3 (0.6 , 0.2 , 0.2) , }) , } , Sphere { center : Vec3 (0.0 , 0.0 , 2.0) , radius : 2.0 , material : Box :: new (Dielectric { index : 1.5 }) , } , Sphere { center : Vec3 (4.0 , 0.0 , 2.0) , radius : 2.0 , material : Box :: new (Metal { albedo : Vec3 (0.85 , 0.9 , 0.7) , fuzz : 0.0 , }) , } ,] ; fn random_material () -> Box < dyn Material > { Box :: new (Lambertian { albedo : Vec3 (0.5 , 0.5 , 0.5) , }) } for _ in 0 .. 500 { let r = 0.4 ; let Vec3 (x , y , _) = random_in_unit_disc () ; let pos = 20.0 * Vec3 (x , y , 0.0) + Vec3 (0.0 , 0.0 , r) ; if spheres . iter () . all (| s | (s . center - pos) . length () >= s . radius + r) { spheres . push (Sphere { center : pos , radius : r , material : random_material () , }) ; } } let world : Vec < Box < dyn Model > > = spheres . into_iter () . map (| s | Box :: new (s) as Box < dyn Model >) . collect () ; Box :: new (world) }
};
}
